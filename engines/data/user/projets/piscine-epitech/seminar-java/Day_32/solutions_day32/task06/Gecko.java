public class Gecko {
    private String name;
    private int age;
    private int energy;
    
    public Gecko() {
        this.name = "Unknown";
        this.age = 0;
        this.energy = 100;
        System.out.println("Hello!");
    }
    
    public Gecko(String name) {
        this.name = name;
        this.age = 0;
        this.energy = 100;
        System.out.println("Hello " + name + "!");
    }
    
    public Gecko(String name, int age) {
        this.name = name;
        this.age = age;
        this.energy = 100;
        System.out.println("Hello " + name + "!");
    }
    
    public Gecko(String name, int age, int energy) {
        this.name = name;
        this.age = age;
        this.energy = energy;
        if (this.energy > 100) this.energy = 100;
        if (this.energy < 0) this.energy = 0;
        System.out.println("Hello " + name + "!");
    }
    
    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }
    
    public int getEnergy() {
        return energy;
    }
    
    public void setEnergy(int energy) {
        this.energy = energy;
        if (this.energy > 100) this.energy = 100;
        if (this.energy < 0) this.energy = 0;
    }

    public void status() {
        switch (age) {
            case 0:
                System.out.println("Unborn Gecko");
                break;
            case 1:
            case 2:
                System.out.println("Baby Gecko");
                break;
            case 3:
            case 4:
            case 5:
            case 6:
            case 7:
            case 8:
            case 9:
            case 10:
                System.out.println("Adult Gecko");
                break;
            case 11:
            case 12:
            case 13:
                System.out.println("Old Gecko");
                break;
            default:
                System.out.println("Impossible Gecko");
                break;
        }
        System.out.println("Energy: " + energy);
    }
    
    public void eat(String food) {
        String foodLower = food.toLowerCase();
        
        if (foodLower.equals("meat")) {
            System.out.println("Yummy!");
            energy += 10;
            if (energy > 100) energy = 100;
        } else if (foodLower.equals("vegetable")) {
            System.out.println("Erk!");
            energy -= 10;
            if (energy < 0) energy = 0;
        } else {
            System.out.println("I can't eat this!");
        }
    }
    
    public void hello(String string) {
        System.out.println("Hello " + string + ", I'm " + getName() + "!");
    }
    
    public void hello(int number) {
        for (int i = 0; i < number; i++) {
            System.out.println("Hello, I'm " + getName() + "!");
        }
    }
}