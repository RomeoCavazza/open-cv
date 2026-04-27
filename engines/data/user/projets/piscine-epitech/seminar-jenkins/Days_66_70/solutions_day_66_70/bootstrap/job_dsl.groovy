job('Disk Space Check') {
    description('Freestyle job that executes df to show disk usage.')
    steps {
        shell('df')
    }
}

job('Daily Dose of Satisfaction') {
    description('Comfort job with a NAME parameter and a friendly message.')

    parameters {
        stringParam('NAME', '', 'Name of the user')
    }

    steps {
        shell('echo "Hello dear $NAME!"')
        shell('date')
        shell('echo "This is your DDoS number $BUILD_NUMBER."')
    }
}