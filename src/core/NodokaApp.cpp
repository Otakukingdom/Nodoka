//
// Created by mistlight on 1/2/2017.
//

#include "NodokaApp.h"
#include <QItemSelection>
#include "PlayerEventHandler.h"

const static char* MAINWINDOW_STYLE =
"QMainWindow {"
        "background-color: #2b4251;"
        "font-family: \"Cabin Medium\";"
        "font-size: 14px;"
        "color: #fff;"
        "}"
        ""
        "QListView {"
        "font-family: \"Raleway Medium\";"
        "font-size: 14px;"
        "border: 0px solid #38302e;"
        "background-color: #fff;"
        "color: #2d3142;"
        "}"
        ""
        "QAbstractScrollArea {"
        "background-color: #91b3bc;"
        "color: #2b4251;"
        "}"
        ""
        "QToolButton {"
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
        ""
;

Core::NodokaApp::NodokaApp() {
    // load fonts
    QFontDatabase::addApplicationFont(":CabinM.ttf");

    // we need this to read settings
    this->setting = new Setting();

    // initialize db backed models
    this->directoryModel = new Directory();
    this->audiobookFileModel = new AudiobookFile();
    this->audiobookModel = new Audiobook(this->audiobookFileModel);

    // initialize player, which will initialize vlc backend related items
    this->player = new Core::ConcretePlayer(this->setting);

    // initialize the ui
    this->mainWindow = new MainWindow(this->directoryModel,
                                      this->audiobookModel,
                                      this->player,
                                      this->setting);

    // initialize event handlers
    this->directoryHandler = new DirectoryHandler(this->audiobookModel, this->audiobookFileModel);
    this->playerEventHandler = new PlayerEventHandler(this->player, this->mainWindow);


    this->setup();
}

void Core::NodokaApp::start() {
    mainWindow->show();
}

void Core::NodokaApp::setup() {
    // set the stylesheet
    this->mainWindow->setStyleSheet(MAINWINDOW_STYLE);

    // set up the listeners for the directory add/remove
    connect(this->directoryModel, &Directory::directoryAdded,
            this->directoryHandler, &DirectoryHandler::handleDirectoryAdded);
    connect(this->directoryModel, &Directory::directoryRemove,
            this->directoryHandler, &DirectoryHandler::handleDirectoryRemoved);


    // we need to register this metatype before using it in signal/slot pattern
    qRegisterMetaType<AudiobookFileProxy>("AudiobookFileProxy");
    qRegisterMetaType<QItemSelection>("QItemSelection");

    // set up the events between playerEvents and mainWindow
    connect(this->playerEventHandler, &PlayerEventHandler::notifyPlayerState,
            this->mainWindow, &MainWindow::playerStateUpdated);
    connect(this->playerEventHandler, &PlayerEventHandler::notifyPlayerTime,
            this->mainWindow, &MainWindow::playerTimeUpdated);
    connect(this->playerEventHandler, &PlayerEventHandler::notifyMediaParsed,
            this->mainWindow, &MainWindow::audiobookFileStateUpdated);
}

Core::NodokaApp::~NodokaApp() {
}
