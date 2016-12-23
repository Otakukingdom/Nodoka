//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MAINWINDOW_H
#define NODOKANATIVE_MAINWINDOW_H


#include <QtWidgets/QWidget>
#include <QMainWindow>
#include "ui_MainWindow.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {

public:
    MainWindow(QWidget *parent = 0);
    virtual ~MainWindow() { };

private:
    Ui::MainWindow *ui;

};


#endif //NODOKANATIVE_MAINWINDOW_H
