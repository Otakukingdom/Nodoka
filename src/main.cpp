#include <QApplication>
#include <QWidget>
#include <QDebug>
#include <src/core/DatabaseConnect.h>
#include <QtWidgets/QMessageBox>
#include "ui-element/MainWindow.h"

int main(int argc, char *argv[]) {

    QApplication app(argc, argv);

    if(!Core::openDb()) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Failed to load the config file");

        // since we failed to load the db, we shouldn't continue
        return 1;
    } else {
        QWidget *widget = new MainWindow();
        widget->show();
    }

    return app.exec();
}
