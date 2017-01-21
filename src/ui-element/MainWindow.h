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
#include <QComboBox>
#include <QTreeWidget>
#include <QStandardItemModel>
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
    void setSpeed(QString speed);


public slots:
    void audiobookFileStateUpdated(AudiobookFileProxy abFile);
    void playerStateUpdated(AudiobookFileProxy abFile, bool isPlaying);
    void playerTimeUpdated(AudiobookFileProxy abFile, long long currentTime);

};

// Stylesheet string to be used in the Audiobook list view
const static char* AB_ITEM_STYLESHEET = ""
        "div {"
        "padding: 3px 5px 3px 5px;"
        "font-family: \"Cabin Medium\";"
        "}"
        ""
        "span.name {"
        "font-family: \"Cabin Bold\";"
        "font-size: 14px;"
        "font-weight: bold;"
        "}"
;

const static char* LIST_VIEW_STYLESHEET = ""
        "QListView {"
        "font-family: \"Raleway Medium\";"
        "font-size: 14px;"
        "border: 0px solid #38302e;"
        "background-color: #91b3bc;"
        "color: #2d3142;"
        "}"
        ""
        "QListView::item:selected {"
        "background-color: #3e3e3b;"
        "}"
        ""
        "QListView::item:selected:active {"
        "background-color: #3e3e3b;"
        "}"
        ""
        "QListView::item:hover {"
        "background-color: #5b7d87;"
        "}"
;

// Stylesheet string to be used in the File list view
const static char* FILE_ITEM_STYLESHEET = "";

#endif //NODOKANATIVE_MAINWINDOW_H
