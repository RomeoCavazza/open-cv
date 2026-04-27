public class NoSuchFoodException extends Exception {
    private static final long serialVersionUID = 1L;
    
    public NoSuchFoodException(String message) {
        super(message);
    }
}
