import java.util.ArrayList;
import java.util.List;
import java.util.Locale;

public class CustomerOrder {
    private Stock stock;
    private List<Food> foodItems;
    private List<Menu<?, ?>> menuItems;

    public CustomerOrder(Stock stock) {
        this.stock = stock;
        this.foodItems = new ArrayList<>();
        this.menuItems = new ArrayList<>();
    }

    public boolean addItem(Food food) throws NoSuchFoodException {
        stock.remove(food.getClass());
        foodItems.add(food);
        return true;
    }

    public boolean removeItem(Food food) {
        if (!foodItems.contains(food)) {
            return false;
        }
        try {
            foodItems.remove(food);
            stock.add(food.getClass());
            return true;
        } catch (NoSuchFoodException e) {
            return false;
        }
    }

    public float getPrice() {
        float total = 0f;
        for (Food item : foodItems) {
            total += item.getPrice();
        }
        for (Menu<?, ?> menu : menuItems) {
            total += menu.getPrice();
        }
        return total;
    }

    public boolean addMenu(Menu<?, ?> menu) throws NoSuchFoodException {
        Class<? extends Food> drinkClass = menu.getDrink().getClass();
        Class<? extends Food> mealClass = menu.getMeal().getClass();
        
        // Retirer le drink d'abord
        if (!stock.remove(drinkClass)) {
            return false;
        }
        
        // Essayer de retirer le meal, restaurer le drink en cas d'échec
        try {
            if (!stock.remove(mealClass)) {
                stock.add(drinkClass); // Restaurer le drink
                return false;
            }
        } catch (NoSuchFoodException e) {
            stock.add(drinkClass); // Restaurer le drink
            throw e;
        }
        
        menuItems.add(menu);
        return true;
    }

    public boolean removeMenu(Menu<?, ?> menu) {
        if (!menuItems.contains(menu)) {
            return false;
        }
        try {
            menuItems.remove(menu);
            stock.add(menu.getDrink().getClass());
            stock.add(menu.getMeal().getClass());
            return true;
        } catch (NoSuchFoodException e) {
            return false;
        }
    }

    public void printOrder() {
        System.out.println("Your order is composed of:");
        
        for (Menu<?, ?> menu : menuItems) {
            String menuType = menu.getClass().getSimpleName();
            System.out.println("- " + menuType + " menu (" + String.format(Locale.US, "%.2f", menu.getPrice()) + " euros)");
            System.out.println("  -> drink: " + menu.getDrink().getClass().getSimpleName());
            System.out.println("  -> meal: " + menu.getMeal().getClass().getSimpleName());
        }
        
        for (Food item : foodItems) {
            float price = item.getPrice();
            String priceStr = String.format(Locale.US, "%.2f", price);
            if (priceStr.endsWith("0")) {
                priceStr = priceStr.substring(0, priceStr.length() - 1);
            }
            System.out.println("- " + item.getClass().getSimpleName() + " (" + priceStr + " euros)");
        }
        
        System.out.println("For a total of " + String.format(Locale.US, "%.2f", getPrice()) + " euros.");
    }
}
