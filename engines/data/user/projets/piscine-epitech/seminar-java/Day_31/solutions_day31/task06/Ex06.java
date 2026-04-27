public class Ex06 {
    public static void sequence(int nbr) {
        if (nbr < 0) {
            return;
        }
        
        String current = "1";
        for (int i = 0; i <= nbr; i++) {
            System.out.println(current);
            if (i < nbr) {
                current = generateNext(current);
            }
        }
    }
    
    private static String generateNext(String previousTerm) {
        int count = 0;
        char prev = '0';
        String ans = "";
        
        for(int i = 0; i < previousTerm.length(); i++) {
            if (previousTerm.charAt(i) == prev) {
                count++;
            } else {
                if (count != 0) {
                    ans += count;
                    ans += prev;
                } 
                prev = previousTerm.charAt(i);
                count = 1;
            }
        }
        if (count != 0) {
            ans += count;
            ans += prev;
        }
        return ans;
    }
}