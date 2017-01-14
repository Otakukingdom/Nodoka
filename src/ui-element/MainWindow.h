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
#include <src/core/Setting.h>
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

    Core::Setting* settings;

    // logical states
    bool isPlaying;
    AudiobookFileProxy currentlyPlayingFile;
    double currentTime;
    void setCurrentlyPlayingFile(AudiobookFileProxy file);

public:
    MainWindow(Directory* directoryModel,
               Audiobook* audiobookModel,
               Core::ConcretePlayer* concretePlayer,
               Core::Setting* setting,
               QWidget *parent = 0);
    virtual ~MainWindow();

    void performSettings();
    void performExit();

    // helper functions
    void setup();
    void loadCurrentAudiobookIfExists();

    void setSelectedFile(QString path);
    void setCurrentTime(long long currentTime);
    void setIsPlaying(bool isPlaying);
    void updateFileView();
    void populateSpeedChoose();

public slots:
    void audiobookFileStateUpdated(AudiobookFileProxy abFile);
    void playerStateUpdated(AudiobookFileProxy abFile, bool isPlaying);
    void playerTimeUpdated(AudiobookFileProxy abFile, long long currentTime);

};


#endif //NODOKANATIVE_MAINWINDOW_H
