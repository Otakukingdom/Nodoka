//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MAINWINDOW_H
#define NODOKANATIVE_MAINWINDOW_H


#include <QtWidgets/QWidget>
#include <QMainWindow>
#include <src/model/Directory.h>
#include <src/model/Audiobook.h>
#include "ui_MainWindow.h"
#include "SettingsForm.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {

private:
    Audiobook* audiobookModel;
    Directory* directoryModel;
    Ui::MainWindow *ui;
    SettingsForm* settingsForm;


public:
    MainWindow(Directory* directoryModel,
               Audiobook* audiobookModel,
               QWidget *parent = 0);
    virtual ~MainWindow();
    void performSettings();
    void performExit();
    void setup();

};


#endif //NODOKANATIVE_MAINWINDOW_H
