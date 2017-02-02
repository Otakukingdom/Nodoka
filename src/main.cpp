#include <QApplication>
#include <QWidget>
#include <src/core/DatabaseConnect.h>
#include <src/core/ConcretePlayer.h>
#include <QtWidgets/QMessageBox>
#include <src/core/NodokaApp.h>
#include <src/simple-lib/RunGuard.h>


int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setOrganizationName("Otakukingdom Co");
    app.setOrganizationDomain("nodoka.otakukingdom.com");
    app.setApplicationName("Nodoka");

    RunGuard guard("Z0DWjf33Am1YeCUdIW7h0vSxjU2RJjZcUqzgG ver0.0.2a");


    if(!guard.tryToRun()) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Cannot launch multiple instances of Nodoka Player");

        return EXIT_FAILURE;
    }



    if(!Core::openDb()) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Failed to load the config file");

        // since we failed to load the db, we shouldn't continue
        return EXIT_FAILURE;
    } else {
        Core::NodokaApp app;

        app.start();
    }

    return app.exec();
}
