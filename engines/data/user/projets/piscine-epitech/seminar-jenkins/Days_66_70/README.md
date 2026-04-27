# Days 66–70 — Notes

- Objectifs Bootstrap: [OBJECTIVES.md](OBJECTIVES.md)
- Objectifs Projet: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day_66_70/](solutions_day_66_70/)

## Objectifs
- Découvrir Jenkins comme plateforme d’automatisation CI/CD.  
- Mettre en place une configuration Jenkins **entièrement déclarative** via JCasC (`my_marvin.yml`).  
- Générer des jobs avec **Job DSL Groovy** (`job_dsl.groovy`) pour compiler, tester et nettoyer des projets C.  
- Définir une stratégie d’authentification et d’autorisations fine basée sur les rôles.  

## Actions
- Configuration globale Jenkins (message système, plugins requis, JCasC activé).  
- Création d’utilisateurs (`Hugo`, `Garance`, `Jeremy`, `Nassim`) alimentés **uniquement** par variables d’environnement.  
- Mise en place d’une stratégie d’autorisations basée sur des rôles (`admin`, `ape`, `gorilla`, `assist`).  
- Création d’un dossier `Tools` contenant les jobs `clone-repository` et `SEED`.  
- Implémentation du job `SEED` qui génère dynamiquement des jobs de build à partir de `GITHUB_NAME` et `DISPLAY_NAME`.  
- Configuration des jobs générés : SCM GitHub, SCM polling, cleanup du workspace, séquence `make fclean && make && make tests_run && make clean`.  

## Leçons
- **JCasC** : un seul YAML peut décrire toute l’instance Jenkins (sécurité, users, plugins, jobs seedés).  
- **Job DSL** : factoriser la création de jobs, éviter la configuration manuelle dans l’interface Jenkins.  
- **Sécurité** : ne jamais hardcoder les mots de passe, tout passer par variables d’environnement.  
- **Rôles** : fine granularité des permissions avec `role-strategy` (admin complet vs rôles pédagogiques/assistants).  
- **CI/CD Epitech-style** : reproduction de la logique de my\_marvin (clonage, build, tests, nettoyage) dans Jenkins.  

---

MY_MARVIN\_ \< CHANGE SIDES AND TAME THE BEAST. /\>

MY_MARVIN Jenkins is a popular Open Source automation platform that can
automate any task you want, from simple periodic cleanups to full
production-scaled deployments.

Therefore, it allows to setup the cornerstone of the DevOps chain: the
Continuous Integration & Continuous Delivery pipeline.

Did you know that the entire Automated Testing system at Epitech is made
with Jenkins? It can simultaneously test \~50 projects in a short period
of time (unless your program is stuck in an infinite loop, but this is
your responsibility!).

Technical formalities In order to be correctly evaluated, you need to
carefully read the following formalities. The project will be entirely
evaluated with Automated Tests. Marvin evaluating your Marvin, doesn't
that sound nice? Automatically testing via an automation platform the
automatic configuration of an automation platform... thanks DevOps! ;)

Configuration as Code

Throughout this project you will use Configuration as Code (a.k.a.
JCasC), which allows you to describe your entire desired Jenkins
configuration in just a single YAML file.

As you might have guessed now, a lot of the developer side of the DevOps
universe is really just configuration files. You can even automate the
automation platform!

You will have to turn in a YAML file called my_marvin.yml, located at
the root of the repository, containing all the necessary configuration
described below.

1

There must be no hardcoded password, all of them must be set by
retrieving the values of the associated environment variables. Any
violation will result in a failure of the entire project. You have been
warned.

Job DSL

You will also have to use the Job Domain Specific Language (a.k.a. Job
DSL) in order to create elements such as jobs. All your DSL scripts must
be centralized into one job_dsl.groovy file located at the root of the
repository. Other Job DSL sources will not be evaluated.

Jenkins and plug-ins' versions

The Jenkins version used for the tests will be the current LTS version.
The plug-ins will be in their latest compatible version with the current
LTS version.

Do not use unnecessary plug-ins, as the virtual Jenkins instance will
only have the required ones installed. If you use unnecessary plug-ins,
the entire DSL correction will fail. You have been warned. Installed
plug-ins: cloudbees-folder, configuration-as-code, credentials, github,
instance-identity, job-dsl, script-security, structs, role-strategy and
ws-cleanup.

Now, onto the practical specifications!

2

Configuration specifications If particular settings or elements are not
specified or addressed in the subject, you are free to do as you please
with them.

Global configuration

3 The instance must be configured with a system message saying "Welcome
to the Chocolatine-Powered Marvin Jenkins Instance.".

Users

3 Signing up must be disallowed. 3 A user named Hugo must be created and
has: -- an id chocolateen; -- a password given by the
USER_CHOCOLATEEN_PASSWORD environment variable. 3 A user named Garance
must be created and has: -- an id vaugie_g; -- a password given by the
USER_VAUGIE_G_PASSWORD environment variable. 3 A user named Jeremy must
be created and has: -- an id i_dont_know; -- a password given by the
USER_I_DONT_KNOW_PASSWORD environment variable. 3 A user named Nassim
must be created and has: -- an id nasso; -- a password given by the
USER_NASSO_PASSWORD environment variable.

Authorization strategy

3 The authorization strategy must be role-based. 3 A global role named
admin must be created and: -- has a "Marvin master" description; -- has
all the permissions; -- is assigned to Hugo.

Do not blindly list all the permissions for the admin role, find the
single right one to give.

3

3 A global role named ape must be created and: -- has a "Pedagogical
team member" description; -- can build a job and see their workspaces;
-- is assigned to Jeremy. 3 A global role named gorilla must be created
and: -- has a "Group Obsessively Researching Innovation Linked to
Learning and Accomplishment" description; -- has the same permissions as
the ape role, plus: \* the ability to create, configure, delete and move
a job, \* cancel builds; -- is assigned to Garance. 3 A global role
named assist must be created and: -- has a "Assistant" description; --
can only view jobs and their workspaces; -- is assigned to Nassim.

3 The specified permissions are the only ones to grant, any
non-specified permission must not be given (unless it is inherently
needed). 3 The specified roles are the only ones to define, no other
role than those can be defined.

Folder

A folder is named Tools and located at the root. It has a "Folder for
miscellaneous tools." description.

Jobs

Each of the following jobs is expected to be enabled and to be a
freestyle job: 3 a job named clone-repository: -- is in the Tools folder
; -- has a GIT_REPOSITORY_URL string parameter with a "Git URL of the
repository to clone" description and no default value ; -- when
executed, clones with Git the repository at the specified URL, using a
single shell command ; -- performs a pre-build workspace cleanup ; -- is
only executed manually. 3 a job named SEED: -- is in the Tools folder ;
4

-- has the following string parameters with no default values: \*
GITHUB_NAME with a "GitHub repository owner/repo_name" description
(e.g.: "EpitechIT31000/chocola ; \* DISPLAY_NAME with a "Display name
for the job" description ; -- when executed, creates a job with the
specifications listed below, using a single job_dsl.groovy file script
execution ; -- is only executed manually. 3 all the jobs that are to be
created by the SEED job: -- are at root ; -- are named according to the
value of the DISPLAY_NAME parameter ; -- have a GitHub project property
pointing at the repository specified by the value of the GITHUB_NAME
parameter ; -- have no parameters ; -- are only executed either by: \* a
SCM poll triggered every minute (which starts a build if there are
changes since the last one); \* manual trigger. -- use the prebuilt Git
SCM system to automatically get the GitHub repository given by the value
of the GITHUB_NAME parameter ; -- perform a pre-build workspace cleanup
; -- when executed, launch each of the following commands in separate
shell script steps:

-   make fclean
-   make
-   make tests_run
-   make clean

You can setup both the GitHub project URL and the Git SCM URL with a
single instruction using the GITHUB_NAME parameter, simplify your job by
trying to find it!

A root element is an element visible on the home page's dashboard.

Tips and tricks 3 You MUST test your configuration and your Job DSL
scripts in order to verify that they are working as intended, you must
NOT just put something in your files and wait for the Automated Tests to
run. If you are that lazy, you will at 100 % fail the project, and you
will have 31 years of bad luck. 3 To facilitate your tests, it is highly
encouraged that you use the first DevOps technology you discovered; it
is WAY more easier to use boxes instead of just manually installing
Jenkins on your computer. Wait, they are not called boxes? Argh, this
whale made me forget their real name... 3 Use your other Epitech
projects to test your SEED job, this is what they are for. ;) 5

v 1.5.1


