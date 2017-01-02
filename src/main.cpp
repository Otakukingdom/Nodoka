#include <QApplication>
#include <QWidget>
#include <QDebug>
#include <src/core/DatabaseConnect.h>
#include <src/core/ConcretePlayer.h>
#include <QtWidgets/QMessageBox>
#include <src/model/Directory.h>
#include <src/core/NodokaApp.h>
#include "ui-element/MainWindow.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);


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
