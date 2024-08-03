import java.security.SecureRandom;
import javax.crypto.Cipher;
import javax.crypto.spec.GCMParameterSpec;
import javax.crypto.spec.SecretKeySpec;

public class Gcm
{

    public static void main(String[] args) throws Exception
    {
	String ct =  encrypt("hello world!","12345");
	String s =  decrypt(ct,"12345");	
        System.out.println("text : "  + s);      	
    }

    private static String decrypt(String ct, String password)  throws Exception
    {
      String r = "";
      byte[] password_byte = make_byte_password(password);      
      String[] a = ct.split("/");
      byte[] iv   =   hex2byte(a[0]);
      byte[] ctext =  hex2byte(a[1] + a[2]);
      
      GCMParameterSpec param_spec = new GCMParameterSpec(128, iv);
      SecretKeySpec key_spec = new SecretKeySpec(password_byte, "AES");
      Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
      cipher.init(Cipher.ENCRYPT_MODE, key_spec, param_spec);

      cipher.init(Cipher.DECRYPT_MODE, key_spec, param_spec);
      byte[] bt = cipher.doFinal(ctext);
      r =  new String(bt);
      return r;
      
    }  
    private static String encrypt(String data, String password)  throws Exception
    {
      String r = "";

      // IV  
      byte[] iv = new byte[12];
      SecureRandom random = new SecureRandom();	
      random.nextBytes(iv);		
      String hex_iv = byte2hex(iv);
      //System.out.println("hex_iv : "  + hex_iv);

      // Password       
      byte[] password_byte = make_byte_password(password);


      // Prepare utils
      byte[] data_byte = data.getBytes();      
      GCMParameterSpec param_spec = new GCMParameterSpec(128, iv);
      SecretKeySpec key_spec = new SecretKeySpec(password_byte, "AES");
      Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
      cipher.init(Cipher.ENCRYPT_MODE, key_spec, param_spec);

      
      // Encrypt 
      byte[] ct = cipher.doFinal(data_byte);


      // Assemble results  
      String hex_ct = byte2hex(ct);		
      String hex_mac =  hex_ct.substring(hex_ct.length() - 32);
      String hex_text = hex_ct.substring(0, hex_ct.length() - 32);
      r = hex_iv + "/" + hex_text + '/' + hex_mac;
      System.out.println("ct : "  + r);      
      return r;
    }							

    private static byte[] make_byte_password(String password)  throws Exception
    {
      // Password       
      byte[] pb = password.getBytes("US-ASCII");
      byte[] password_byte = new byte[16];
      for (int i = 0; i < password_byte.length; i++)
      {
        if(pb.length > i)
        {
          password_byte[i] = pb[i];
        }		     
      }
      return password_byte;
    }


    private static void printbyte(byte[] bytes)  throws Exception
    {
      for(byte b:bytes)
      {	  
        System.out.println(b);
      }	  
    }
    

    public static String byte2hex(byte[] byteArray)
    {
      String hex = "";
      for (byte i : byteArray)
      {
        hex += String.format("%02X", i);
      }
      return hex;
    }
  

    public static byte[] hex2byte(String s)
    {
     byte[] ans = new byte[s.length() / 2];
     for (int i = 0; i < ans.length; i++)
     {
       int index = i * 2;
       int val = Integer.parseInt(s.substring(index, index + 2), 16);
       ans[i] = (byte)val;
     }
     return ans;
    }


    private static void test()  throws Exception
    {
	String data = "hello world";
	byte[] data_byte = data.getBytes();
	//printbyte(data_byte);
        String password = "12345";
        byte[] pb = password.getBytes("US-ASCII");
        byte[] password_byte = new byte[16];
        for (int i = 0; i < password_byte.length; i++) {
          if(pb.length > i)
	  {
	    password_byte[i] = pb[i];
          }		     
        }
	//printbyte(password_byte);

        System.out.println("data : " + data);
        System.out.println("password : " + password);	


        byte[] iv = hex2byte("23e73caa6570642b43a5f307");
        //byte[] iv = new byte[12];
        //SecureRandom random = new SecureRandom();	
        //random.nextBytes(iv);		
        //for(byte b:iv)
        //  System.out.println(b);

        GCMParameterSpec param_spec = new GCMParameterSpec(128, iv);
        SecretKeySpec key_spec = new SecretKeySpec(password_byte, "AES");
	
        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.ENCRYPT_MODE, key_spec, param_spec);

        byte[] ct = cipher.doFinal(data_byte);
	String x = byte2hex(ct);		
        System.out.println("hex : "  +x);


	byte[] ct2 = hex2byte("99b1ddeed7fa1cb52146edaa995a2c1dfdeb7bb89fa09d76b4da39"); 
        cipher.init(Cipher.DECRYPT_MODE, key_spec, param_spec);
        byte[] bt = cipher.doFinal(ct2);
	String text =  new String(bt);
        System.out.println("Text : " + text);

    }							    
}
