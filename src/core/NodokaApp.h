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
        "font-family: \"Cabin Medium\";"
        "font-size: 14px;"
        "}"
        ""
        "QWidget#topHorizontalWidget {"
        "background-color: #FEDB53;"
        "}"
        ""
        "QWidget#playerVerticalWidget {"
        "background-color: #414141;"
        "color: #eee;"
        "}"
        ""
        "QToolButton{"
        "font-family: \"Raleway Medium\";"
        "font-size: 15px;"
        "background-color:#45415e;"
        "color: #91b3bc;"
        "border-radius: 3px;"
        "padding: 3px 6px 3px 6px;"
        "}"
        ""
        "QComboBox#speedChooser {"
        "color: #2e323c;"
        "background-color: #4f5d75;"
        "border-radius: 5px;"
        "padding: 5px 5px 5px 5px;"
        "}"
        ""
        "QPushButton {"
        "background-color: #c2d7dd;"
        "padding: 5px 5px 5px 5px;"
        "border-radius: 3px;"
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
