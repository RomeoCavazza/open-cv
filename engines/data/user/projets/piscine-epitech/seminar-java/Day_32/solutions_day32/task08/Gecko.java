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
    
    public void work() {
        if (energy >= 25) {
            System.out.println("I'm working T.T");
            energy -= 9;
            if (energy < 0) energy = 0;
        } else {
            System.out.println("Heyyy I'm too sleepy, better take a nap!");
            energy += 50;
            if (energy > 100) energy = 100;
        }
    }
    
    public void fraternize(Gecko other) {
        if (this.energy >= 30 && other.energy >= 30) {
            System.out.println("I'm going to drink with " + other.getName() + "!");
            System.out.println("I'm going to drink with " + this.getName() + "!");
            this.energy -= 30;
            other.energy -= 30;
            if (this.energy < 0) this.energy = 0;
            if (other.energy < 0) other.energy = 0;
        } else if (this.energy < 30 && other.energy < 30) {
            System.out.println("Not today!.");
            System.out.println("Not today!.");
        } else if (this.energy < 30) {
            System.out.println("Sorry " + other.getName() + ", I'm too tired to go out tonight.");
            System.out.println("Oh! That's too bad, another time then!");
        } else {
            System.out.println("Sorry " + this.getName() + ", I'm too tired to go out tonight.");
            System.out.println("Oh! That's too bad, another time then!");
        }
    }
    
    public void fraternize(Snake snake) {
        if (this.energy >= 10) {
            System.out.println("LET'S RUN AWAY!!!");
            this.energy = 0;
        } else {
            System.out.println("...");
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
