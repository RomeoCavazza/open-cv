import java.util.ArrayList;

public class Team {
    private String name;
    private ArrayList<Astronaut> members;
    
    public Team(String name) {
        this.name = name;
        this.members = new ArrayList<>();
    }
    
    public String getName() {
        return name;
    }
    
    public void add(Astronaut astronaut) {
        members.add(astronaut);
    }
    
    public void remove(Astronaut astronaut) {
        members.remove(astronaut);
    }
    
    public int countMembers() {
        return members.size();
    }
    
    public void showMembers() {
        if (members.isEmpty()) {
            return;
        }
        
        StringBuilder result = new StringBuilder(name + ": ");
        for (int i = 0; i < members.size(); i++) {
            Astronaut astronaut = members.get(i);
            if (i > 0) {
                result.append(", ");
            }
            result.append(astronaut.getName());
            if (astronaut.getDestination() != null) {
                result.append(" on mission");
            } else {
                result.append(" on standby");
            }
        }
        result.append(".");
        System.out.println(result.toString());
    }
    
    public void doActions() {
        System.out.println(name + ": Nothing to do.");
    }
    
    public void doActions(planet.Mars planetMars) {
        for (Astronaut astronaut : members) {
            astronaut.doActions(planetMars);
        }
    }
    
    public void doActions(chocolate.Mars chocolateMars) {
        for (Astronaut astronaut : members) {
            astronaut.doActions(chocolateMars);
        }
    }
    
    public void doActions(planet.moon.Phobos phobos) {
        for (Astronaut astronaut : members) {
            astronaut.doActions(phobos);
        }
    }
}
