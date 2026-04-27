package Observer;

public class Customer implements Observer {
    
    @Override
    public void update(Observable observable) {
        if (observable instanceof Order) {
            Order order = (Order) observable;
            String position = order.getPosition();
            String destination = order.getDestination();
            int time = order.getTimeBeforeArrival();
            System.out.println("Position (" + position + "), " + time + " minutes before arrival at " + destination + ".");
        }
    }
}
