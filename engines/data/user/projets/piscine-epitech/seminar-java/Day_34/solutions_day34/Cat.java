public class Cat extends Animal {
    private String color;

    public Cat(String name) {
        this(name, "grey");
    }

    public Cat(String name, String color) {
        super(name, 4, Type.MAMMAL);
        this.color = color;
        System.out.println(name + ": MEEEOOWWWW");
    }

    public void meow() {
        System.out.println(name + " the " + color + " kitty is meowing.");
    }

    public String getColor() {
        return color;
    }
}