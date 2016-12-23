#include <QApplication>
#include <QtWidgets/QPushButton>
#include <zconf.h>
#include <assert.h>

int main(int argc, char *argv[]) {
    QApplication a(argc, argv);

    QPushButton button ("Hello world !");
    button.show();

    return a.exec();
}