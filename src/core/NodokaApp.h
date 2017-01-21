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
        "background-color: #ebf2f4;"
        "font-family: \"Cabin Medium\";"
        "font-size: 14px;"
        "color: #2e323c;"
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
        "QMenuBar::item {"
        "font-family: \"Cabin Medium\";"
        "}"
        ""
        "QPushButton {"
        "background-color: #c2d7dd;"
        "padding: 5px 5px 5px 5px;"
        "border-radius: 3px;"
        "}"
        ""
        "QLabel {"
        "color: #2e323c;"
        "}"
        ""
        "QLabel#fileTitle, QLabel#abTitle {"
        "font-family: \"Raleway Medium\";"
        "font-size: 20px;"
        "}"
        ""
        "QSlider#volumeSlider::groove:horizontal {"
        "margin: 2px 0;"
        "background-color: #45415e;"
        "height: 5px;"
        "}"
        ""
        "QSlider#volumeSlider::handle:horizontal {"
        "width: 10px;"
        "margin: -10 0;"
        "background-color: #5b7d87;"
        "}"
        ""
        "QSlider#progressSlider::groove:horizontal {"
        "margin: 2px 0;"
        "background-color: #45415e;"
        "height: 10px;"
        "}"
        ""
        "QSlider#progressSlider::handle:horizontal {"
        "width: 5px;"
        "margin: -10 0;"
        "background-color: #5b7d87;"
        "}"
        ""
        "QScrollBar:horizontal {"
        "border: 2px solid grey;"
        "background: #32CC99;"
        "height: 15px;"
        "margin: 0px 20px 0 20px;"
        "}"
        ""
        "QScrollBar::handle:horizontal {"
        "background: white;"
        "min-width: 20px;"
        "}"
        ""
        "QScrollBar::add-line:horizontal {"
        "border: 2px solid grey;"
        "background: #2e323c;"
        "width: 20px;"
        "subcontrol-position: right;"
        "subcontrol-origin: margin;"
        "}"
        ""
        "QScrollBar::sub-line:horizontal {"
        "border: 2px solid grey;"
        "background: #2e323c;"
        "width: 20px;"
        "subcontrol-position: left;"
        "subcontrol-origin: margin;"
        "}"
        ""
        "QScrollBar:left-arrow:horizontal, QScrollbar::right-arrow:horizontal {"
        "border: 2px solid grey;"
        "width: 3px;"
        "height: 3px;"
        "background: white;"
        "}"
        ""
        "QScrollBar::add-page:horizontal, QScrollBar::sub-page:horizontal {"
        "background: none;"
        "}"
;


#endif //NODOKANATIVE_NODOKAAPP_H
