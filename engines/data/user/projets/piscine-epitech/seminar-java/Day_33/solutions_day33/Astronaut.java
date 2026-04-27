public class Astronaut {
    private String name;
    private int snacks;
    private String destination;
    private int id;
    private static int nextId = 0;
    
    public Astronaut(String name) {
        this.name = name;
        this.snacks = 0;
        this.destination = null;
        this.id = nextId++;
        System.out.println(name + " ready for launch!");
    }
    
    public String getName() {
        return name;
    }
    
    public int getSnacks() {
        return snacks;
    }
    
    public String getDestination() {
        return destination;
    }
    
    public int getId() {
        return id;
    }
    
    public void doActions() {
        System.out.println(name + ": Nothing to do.");
        if (destination == null) {
            System.out.println(name + ": I may have done nothing, but I have " + snacks + " Mars to eat at least!");
        }
    }
    
    public void doActions(planet.Mars planetMars) {
        System.out.println(name + ": Started a mission!");
        this.destination = planetMars.getLandingSite();
        if (destination == null) {
            System.out.println(name + ": I may have done nothing, but I have " + snacks + " Mars to eat at least!");
        }
    }
    
    public void doActions(chocolate.Mars chocolateMars) {
        System.out.println(name + ": Thanks for this Mars number " + chocolateMars.getId());
        this.snacks++;
        if (destination == null) {
            System.out.println(name + ": I may have done nothing, but I have " + snacks + " Mars to eat at least!");
        }
    }
    
    public void doActions(planet.moon.Phobos phobos) {
        System.out.println(name + ": Started a mission!");
        this.destination = phobos.getLandingSite();
        if (destination == null) {
            System.out.println(name + ": I may have done nothing, but I have " + snacks + " Mars to eat at least!");
        }
    }
}
