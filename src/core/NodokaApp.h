//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_NODOKAAPP_H
#define NODOKANATIVE_NODOKAAPP_H


#include "ConcretePlayer.h"
#include "src/event-handler/PlayerEventHandler.h"
#include "Setting.h"
#include "ScanPlayer.h"
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
        ScanPlayer *scanPlayer;

        Directory* directoryModel;
        MainWindow* mainWindow;
        DirectoryHandler* directoryHandler;
        Audiobook* audiobookModel;
        AudiobookFile* audiobookFileModel;
        Setting* setting;
        std::shared_ptr<ProxyManager> proxyManager;

        PlayerEventHandler* playerEventHandler;

        //private helper function, used to set up the event listeners
        void setup();

        QThreadPool* scanThread;

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
        "background-color:#858585;"
        "color: #fff;"
        "border-radius: 3px;"
        "padding: 3px 6px 3px 6px;"
        "}"
        ""
        "QComboBox#speedChooser {"
        "font-family: \"Source Sans Pro\";"
        "font-size: 14px;"
        "color: #515151;"
        "background-color: #f5f5f5;"
        "margin-right: 15px;"
        "margin-left: 5px;"
        "padding-left: 5px;"
        "border-radius: 5px;"
        "}"
        ""
        "QComboBox#speedChooser::drop-down {"
        "color: #515151;"
        "background-color: #f5f5f5;"
        "border-top-right-radius: 5px;"
        "border-bottom-right-radius: 5px;"
        "}"
        ""
        "QComboBox#speedChooser::down-arrow {"
        "image: url(:/icons/misc/downarrow.png);"
        "}"
        ""
        "QComboBox#speedChooser QAbstractItemView {"
        "border: 0;"
        "selection-background-color: #DCE9BE;"
        "selection-color: #515151;"
        "}"
        ""
        "QLabel#currentlyPlayingLabel {"
        "font-size: 12px;"
        "font-family: \"Roboto\";"
        "background-color: #F5F5F5;"
        "color: #25232d;"
        "border-radius: 5px;"
        "margin: 25px 15px 15px 15px;"
        "padding: 10px 10px 10px 10px;"
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
        ""
        "QSlider#progressSlider::groove {"
        "margin-left: 15px;"
        "margin-right: 15px;"
        "height: 15px;"
        "background-color: #dcdcdd;"
        "border-radius: 7px;"
        "}"
        ""
        "QSlider#volumeSlider::groove {"
        "height: 15px;"
        "background-color: #dcdcdd;"
        "border-radius: 7px;"
        "}"
        ""
        "QSlider::handle {"
        "background-color: #46494c;"
        "width: 15px;"
        "border-radius: 7px;"
        "}"
        ""
        "QSlider#progressSlider:sub-page{"
        "background-color: #FEDB53;"
        "margin-left: 15px;"
        "border-top-left-radius: 7px;"
        "border-bottom-left-radius: 7px;"
        "}"
        ""
        "QSlider#volumeSlider:sub-page{"
        "background-color: #FEDB53;"
        "border-top-left-radius: 7px;"
        "border-bottom-left-radius: 7px;"
        "}"
        ""
        ""
        "QPushButton#playButton {"
        "background-color: #fff;"
        "border-radius: 15px;"
        "}"
        ""
        "QMenu {"
        "font-family: \"Source Sans Pro\";"
        "background-color:#858585;"
        "color: #fff;"
        "}"
        ""
        "QMenu:selected {"
        "background-color:#FEDB53;"
        "color: #000;"
        "}"

;


#endif //NODOKANATIVE_NODOKAAPP_H
