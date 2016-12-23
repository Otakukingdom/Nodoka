#include <QApplication>
#include <QWidget>
#include <QDebug>
#include "ui-element/MainWindow.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QWidget *widget = new MainWindow();
    widget->show();

    return app.exec();
}
