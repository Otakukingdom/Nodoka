//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MAINWINDOW_H
#define NODOKANATIVE_MAINWINDOW_H


#include <QtWidgets/QWidget>
#include <QMainWindow>
#include <src/model/Directory.h>
#include "ui_MainWindow.h"
#include "SettingsForm.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {

public:
    MainWindow(Directory* directoryModel,
               QWidget *parent = 0);
    virtual ~MainWindow();
    void performSettings();
    void performExit();
    void setup();

private:
    Directory* directoryModel;
    Ui::MainWindow *ui;
    SettingsForm* settingsForm;

};


#endif //NODOKANATIVE_MAINWINDOW_H
