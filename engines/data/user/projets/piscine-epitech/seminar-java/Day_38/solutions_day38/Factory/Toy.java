package Factory;

public abstract class Toy {
    private String title;
    
    public String getTitle() {
        return title;
    }
    
    protected void setTitle(String title) {
        this.title = title;
    }
}
