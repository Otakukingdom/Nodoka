#include <QApplication>
#include <QWidget>
#include <QDebug>
#include <src/core/DatabaseConnect.h>
#include <src/core/ConcretePlayer.h>
#include <QtWidgets/QMessageBox>
#include <src/model/Directory.h>
#include "ui-element/MainWindow.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    // initialize player, which will initialize vlc backend related items
    auto player = new Core::ConcretePlayer();

    if(!Core::openDb()) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Failed to load the config file");

        // since we failed to load the db, we shouldn't continue
        return EXIT_FAILURE;
    } else {
        // initialize db backed models
        auto directoryModel = new Directory();


        QWidget *widget = new MainWindow(directoryModel);
        widget->show();
    }

    return app.exec();
}
