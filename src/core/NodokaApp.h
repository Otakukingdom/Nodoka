//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_NODOKAAPP_H
#define NODOKANATIVE_NODOKAAPP_H


#include "ConcretePlayer.h"
#include "PlayerEventHandler.h"
#include "Setting.h"
#include <QFontDatabase>
#include <src/model/Directory.h>
#include <src/model/Audiobook.h>
#include <src/model/AudiobookFile.h>
#include <src/ui-element/MainWindow.h>
#include <src/event-handler/DirectoryHandler.h>

namespace Core {

    class NodokaApp : public QObject {
    Q_OBJECT
    private:
        ConcretePlayer *player;
        Directory* directoryModel;
        MainWindow* mainWindow;
        DirectoryHandler* directoryHandler;
        Audiobook* audiobookModel;
        AudiobookFile* audiobookFileModel;
        Setting* setting;

        PlayerEventHandler* playerEventHandler;

        //private helper function, used to set up the event listeners
        void setup();

    public:
        NodokaApp();
        ~NodokaApp();
        void start();
    };

}

const static char* MAINWINDOW_STYLE = "QMainWindow {"
        "font-family: \"Source Sans Pro\";"
        "font-size: 14px;"
        "}"
        ""
        "QWidget#topHorizontalWidget {"
        "background-color: #FEDB53;"
        "}"
        ""
        "QWidget#playerVerticalWidget {"
        "font-family: \"Source Sans Pro\";"
        "background-color: #414141;"
        "color: #eee;"
        "}"
        ""
        "QToolButton{"
        "font-family: \"Source Sans Pro\";"
        "font-size: 15px;"
        "background-color:#1c140d;"
        "color: #f2e9e1;"
        "border-radius: 3px;"
        "padding: 3px 6px 3px 6px;"
        "}"
        ""
        "QComboBox#speedChooser {"
        "font-family: \"Source Sans Pro\";"
        "font-size: 14px;"
        "color: #515151;"
        "background-color: #f5f5f5;"
        "border-radius: 5px;"
        "padding: 5px 5px 5px 5px;"
        "margin-right: 10px;"
        "}"
        ""
        "QComboBox#speedChooser::drop-down {"
        "color: #515151;"
        "background-color: #f5f5f5;"
        "}"
        ""
        "QPushButton {"
        "background-color: #c2d7dd;"
        "padding: 5px 5px 5px 5px;"
        "border-radius: 3px;"
        "}"
        ""
        "QLabel#currentlyPlayingLabel {"
        "font-size: 20px;"
        "font-family: \"Roboto\";"
        "background-color: #9da9b5;"
        "color: #000;"
        "border-radius: 5px;"
        "margin: 15px 15px 15px 15px;"
        "padding: 10px 10px 10px 10px;"
        "}"
        ""
        "QLabel#timeLabel {"
        "font-family: \"Roboto Mono\";"
        "margin-right: 5px;"
        "}"
        ""
        "QScrollBar::add-line {"
        "background: transparent;"
        "}"
        ""
        "QScrollBar::add-page {"
        "background: transparent;"
        "}"
        ""
        "QScrollBar::sub-line {"
        "background: transparent;"
        "}"
        ""
        "QScrollBar::sub-page {"
        "background: transparent;"
        "}"
        ""
        "QScrollBar::corner {"
        "background: #000;"
        "}"
        ""
        "QScrollBar:vertical {"
        "border: 0px solid;"
        "background: transparent;"
        "}"
        ""
        "QScrollBar:vertical:hover {"
        "border: 0px solid;"
        "}"
        ""
        "QScrollBar::handle:vertical {"
        "background: grey;"
        "width: 10px;"
        "border-radius: 5px;"
        "min-height: 20px;"
        "}"
;


#endif //NODOKANATIVE_NODOKAAPP_H
