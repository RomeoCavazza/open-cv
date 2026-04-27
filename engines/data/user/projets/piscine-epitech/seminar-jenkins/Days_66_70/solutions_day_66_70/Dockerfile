FROM jenkins/jenkins:lts

ENV JAVA_OPTS -Djenkins.install.runSetupWizard=false

RUN jenkins-plugin-cli --plugins cloudbees-folder configuration-as-code credentials github instance-identity job-dsl script-security structs role-strategy ws-cleanup