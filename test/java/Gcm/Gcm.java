import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;
import java.util.Arrays;
import javax.crypto.Cipher;
import javax.crypto.Mac;
import javax.crypto.spec.GCMParameterSpec;
import javax.crypto.spec.SecretKeySpec;

public class Gcm
{
    private static final String VERSION = "v2";
    private static final String KDF = "pbkdf2-sha256";
    private static final int ITERATIONS = 600000;
    private static final int SALT_LEN = 16;
    private static final int NONCE_LEN = 12;
    private static final int KEY_LEN = 16;
    private static final int TAG_LEN = 16;
    private static final int MAX_ITERATIONS = 2000000;

    public static void main(String[] args) throws Exception
    {
        if (args.length == 0)
        {
            String ct = encrypt("hello world!", "12345");
            System.out.println("ct : " + ct);
            System.out.println("text : " + decrypt(ct, "12345"));
            return;
        }

        if (args.length != 3 || (!"encrypt".equals(args[0]) && !"decrypt".equals(args[0])))
        {
            System.err.println("usage: java Gcm encrypt|decrypt <password> <text>");
            System.exit(2);
        }

        if ("encrypt".equals(args[0]))
        {
            System.out.println(encrypt(args[2], args[1]));
        }
        else
        {
            System.out.println(decrypt(args[2], args[1]));
        }
    }

    private static String encrypt(String data, String password) throws Exception
    {
        SecureRandom random = new SecureRandom();
        byte[] salt = new byte[SALT_LEN];
        byte[] nonce = new byte[NONCE_LEN];
        random.nextBytes(salt);
        random.nextBytes(nonce);

        byte[] key = deriveKey(password, salt, ITERATIONS);
        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.ENCRYPT_MODE, new SecretKeySpec(key, "AES"), new GCMParameterSpec(128, nonce));
        byte[] encrypted = cipher.doFinal(data.getBytes(StandardCharsets.UTF_8));

        byte[] ciphertext = Arrays.copyOfRange(encrypted, 0, encrypted.length - TAG_LEN);
        byte[] tag = Arrays.copyOfRange(encrypted, encrypted.length - TAG_LEN, encrypted.length);

        return VERSION + "/" + KDF + "/" + ITERATIONS + "/" + byte2hex(salt) + "/"
            + byte2hex(nonce) + "/" + byte2hex(ciphertext) + "/" + byte2hex(tag);
    }

    private static String decrypt(String ct, String password) throws Exception
    {
        String[] a = ct.split("/");
        if (a.length == 3)
        {
            return decryptV1(a, password);
        }
        if (a.length != 7 || !VERSION.equals(a[0]) || !KDF.equals(a[1]))
        {
            throw new IllegalArgumentException("unsupported format");
        }

        int iterations = Integer.parseInt(a[2]);
        if (iterations <= 0 || iterations > MAX_ITERATIONS)
        {
            throw new IllegalArgumentException("invalid iterations");
        }

        byte[] salt = hex2byte(a[3]);
        byte[] nonce = hex2byte(a[4]);
        byte[] ciphertext = hex2byte(a[5]);
        byte[] tag = hex2byte(a[6]);
        if (salt.length != SALT_LEN || nonce.length != NONCE_LEN || tag.length != TAG_LEN)
        {
            throw new IllegalArgumentException("invalid lengths");
        }

        byte[] key = deriveKey(password, salt, iterations);
        byte[] encrypted = new byte[ciphertext.length + tag.length];
        System.arraycopy(ciphertext, 0, encrypted, 0, ciphertext.length);
        System.arraycopy(tag, 0, encrypted, ciphertext.length, tag.length);

        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"), new GCMParameterSpec(128, nonce));
        return new String(cipher.doFinal(encrypted), StandardCharsets.UTF_8);
    }

    private static String decryptV1(String[] a, String password) throws Exception
    {
        byte[] iv = hex2byte(a[0]);
        byte[] ciphertext = hex2byte(a[1]);
        byte[] tag = hex2byte(a[2]);
        if (iv.length != NONCE_LEN || tag.length != TAG_LEN)
        {
            throw new IllegalArgumentException("invalid lengths");
        }

        byte[] key = legacyKey(password);
        byte[] encrypted = new byte[ciphertext.length + tag.length];
        System.arraycopy(ciphertext, 0, encrypted, 0, ciphertext.length);
        System.arraycopy(tag, 0, encrypted, ciphertext.length, tag.length);

        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"), new GCMParameterSpec(128, iv));
        return new String(cipher.doFinal(encrypted), StandardCharsets.UTF_8);
    }

    private static byte[] deriveKey(String password, byte[] salt, int iterations) throws Exception
    {
        byte[] passwordBytes = password.getBytes(StandardCharsets.UTF_8);
        if (passwordBytes.length == 0)
        {
            passwordBytes = new byte[] {0};
        }

        Mac mac = Mac.getInstance("HmacSHA256");
        mac.init(new SecretKeySpec(passwordBytes, "HmacSHA256"));

        int hLen = mac.getMacLength();
        int blocks = (KEY_LEN + hLen - 1) / hLen;
        byte[] dk = new byte[blocks * hLen];
        byte[] block = new byte[salt.length + 4];
        System.arraycopy(salt, 0, block, 0, salt.length);

        for (int i = 1; i <= blocks; i++)
        {
            block[salt.length] = (byte) (i >>> 24);
            block[salt.length + 1] = (byte) (i >>> 16);
            block[salt.length + 2] = (byte) (i >>> 8);
            block[salt.length + 3] = (byte) i;

            byte[] u = mac.doFinal(block);
            byte[] t = u.clone();
            for (int j = 1; j < iterations; j++)
            {
                u = mac.doFinal(u);
                for (int k = 0; k < t.length; k++)
                {
                    t[k] ^= u[k];
                }
            }
            System.arraycopy(t, 0, dk, (i - 1) * hLen, hLen);
        }

        return Arrays.copyOf(dk, KEY_LEN);
    }

    private static byte[] legacyKey(String password)
    {
        byte[] pb = password.getBytes(StandardCharsets.UTF_8);
        byte[] key = new byte[16];
        for (int i = 0; i < key.length; i++)
        {
            if (pb.length > i)
            {
                key[i] = pb[i];
            }
        }
        return key;
    }

    public static String byte2hex(byte[] byteArray)
    {
        StringBuilder hex = new StringBuilder(byteArray.length * 2);
        for (byte b : byteArray)
        {
            hex.append(String.format("%02x", b & 0xff));
        }
        return hex.toString();
    }

    public static byte[] hex2byte(String s)
    {
        if ((s.length() & 1) != 0)
        {
            throw new IllegalArgumentException("odd hex length");
        }
        byte[] ans = new byte[s.length() / 2];
        for (int i = 0; i < ans.length; i++)
        {
            int index = i * 2;
            int val = Integer.parseInt(s.substring(index, index + 2), 16);
            ans[i] = (byte) val;
        }
        return ans;
    }
}
