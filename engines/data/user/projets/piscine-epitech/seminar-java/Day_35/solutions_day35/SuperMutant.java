public class SuperMutant extends Monster {
    private static int idCounter = 0;

    public SuperMutant() {
        super("SuperMutant #" + (++idCounter), 170, 20);
        System.out.println(getName() + ": Roaarrr!");
    }

    public int getDamage() {
        return 60;
    }

    public int getApcost() {
        return 20;
    }

    public void recoverAP() {
        super.recoverAP();
        hp += 10;
        if (hp > 170) {
            hp = 170;
        }
    }
}