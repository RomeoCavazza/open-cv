import java.lang.reflect.Method;
import java.lang.reflect.Modifier;

public class TestRunner {
    public static void runTests(Class<?> targetClass) {
        Object instance = null;
        try {
            try {
                instance = targetClass.getDeclaredConstructor().newInstance();
            } catch (NoSuchMethodException e) {
            }

            Method[] methods = targetClass.getDeclaredMethods();

            for (Method method : methods) {
                if (method.isAnnotationPresent(BeforeClass.class)) {
                    invokeMethod(method, targetClass, instance);
                }
            }

            for (Method method : methods) {
                Test testAnnot = method.getAnnotation(Test.class);
                if (testAnnot != null && testAnnot.enabled()) {
                    for (Method m : methods) {
                        if (m.isAnnotationPresent(Before.class)) {
                            invokeMethod(m, targetClass, instance);
                        }
                    }

                    System.out.println(testAnnot.name());
                    invokeMethod(method, targetClass, instance);

                    for (Method m : methods) {
                        if (m.isAnnotationPresent(After.class)) {
                            invokeMethod(m, targetClass, instance);
                        }
                    }
                }
            }

            for (Method method : methods) {
                if (method.isAnnotationPresent(AfterClass.class)) {
                    invokeMethod(method, targetClass, instance);
                }
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    private static void invokeMethod(Method method, Class<?> targetClass, Object instance) throws Exception {
        if (!method.canAccess(Modifier.isStatic(method.getModifiers()) ? null : instance)) {
            method.setAccessible(true);
        }
        if (Modifier.isStatic(method.getModifiers())) {
            method.invoke(null);
        } else {
            if (instance == null) {
                throw new IllegalStateException("No default constructor to instantiate " + targetClass.getName());
            }
            method.invoke(instance);
        }
    }
}
