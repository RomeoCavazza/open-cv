public class Mars {
    private int id;
    private static int nextId = 0;
    
    public Mars() {
        this.id = nextId++;
    }
    
    public int getId() {
        return id;
    }
}
