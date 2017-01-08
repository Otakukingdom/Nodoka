//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MAINWINDOW_H
#define NODOKANATIVE_MAINWINDOW_H


#include <QtWidgets/QWidget>
#include <QMainWindow>
#include <QAbstractItemView>
#include <src/model/Directory.h>
#include <src/model/Audiobook.h>
#include <src/model/FileDisplayModel.h>
#include <src/core/ConcretePlayer.h>
#include "ui_MainWindow.h"
#include "SettingsForm.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {

private:
    Audiobook* audiobookModel;
    FileDisplayModel* fileDisplayModel;
    Core::ConcretePlayer* concretePlayer;
    Directory* directoryModel;
    Ui::MainWindow *ui;
    SettingsForm* settingsForm;

    // logical states
    bool isPlaying;
    AudiobookFileProxy currentlyPlayingFile;
    double currentTime;


public:
    MainWindow(Directory* directoryModel,
               Audiobook* audiobookModel,
               Core::ConcretePlayer* concretePlayer,
               QWidget *parent = 0);
    virtual ~MainWindow();
    void performSettings();
    void performExit();
    void setup();

    void setCurrentlyPlayingFile(AudiobookFileProxy file);
    void setCurrentTime(double currentTime);
    void setIsPlaying(bool isPlaying);

public slots:
    void playerStateUpdated(AudiobookFileProxy abFile, bool isPlaying);
    void playerTimeUpdated(AudiobookFileProxy abFile, double currentTime);

};


#endif //NODOKANATIVE_MAINWINDOW_H
