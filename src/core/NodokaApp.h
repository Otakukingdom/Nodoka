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
        "background-color: #2b4251;"
        "font-family: \"Cabin Medium\";"
        "font-size: 14px;"
        "color: #fff;"
        "}"
        ""
        "QListView#audiobookView, QListView {"
        "font-family: \"Raleway Medium\";"
        "font-size: 14px;"
        "border: 0px solid #38302e;"
        "background-color: #91b3bc;"
        "color: #2d3142;"
        "}"
        ""
        "QListView::item:selected, QListView::item:selected#audiobookView{"
        "background-color: #2e323c;"
        "color: #fff;"
        "}"
        ""
        "QListView::item:selected:active, QListView::item:selected:active#audiobookView{"
        "background-color: #2e323c;"
        "color: #fff;"
        "}"
        ""
        "QListView::item:hover, QListView::item:hover#audiobookView{"
        "background-color: #5b7d87;"
        "}"
        ""
        "QAbstractScrollArea, QAbstractScrollArea#audiobookView {"
        "background-color: #91b3bc;"
        "color: #2b4251;"
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
        ""
        "QMenuBar::item {"
        "font-family: \"Cabin Medium\";"
        "}"
        ""
        "QPushButton {"
        "background-color: #bfc0c0;"
        "color: #91b3bc;"
        "border-radius: 3px;"
        "}"
        ""
        "QLabel {"
        "color: #fff;"
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
        "height: 5px;"
        "}"
        ""
        "QSlider#progressSlider::handle:horizontal {"
        "width: 10px;"
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
