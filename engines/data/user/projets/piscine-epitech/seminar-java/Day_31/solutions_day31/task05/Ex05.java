import java.util.ArrayList;

public class Ex05 {
    public static ArrayList<String> myGetArgs(String... var) {
        ArrayList<String> args = new ArrayList<>();
        for (String arg : var) {
            args.add(arg);
        }
        return args;
    }
}