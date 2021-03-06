//
// Created by mistlight on 1/2/2017.
//

#include "NodokaApp.h"
#include <QItemSelection>
#include <src/core/tasks/InitialScanTask.h>
#include "src/event-handler/PlayerEventHandler.h"
#include "ScanPlayer.h"
#include "DatabaseConnect.h"
#include <memory>

Core::NodokaApp::NodokaApp(QObject* parent) : QObject(parent) {
    // load fonts
    QFontDatabase::addApplicationFont(":RobotoM.ttf");
    QFontDatabase::addApplicationFont(":RobotoB.ttf");
    QFontDatabase::addApplicationFont(":RobotoR.ttf");
    QFontDatabase::addApplicationFont(":RobotoI.ttf");
    QFontDatabase::addApplicationFont(":RobotoMonoR.ttf");
    QFontDatabase::addApplicationFont(":SourceB.ttf");
    QFontDatabase::addApplicationFont(":SourceR.ttf");

    QCoreApplication::setAttribute(Qt::AA_UseStyleSheetPropagationInWidgetStyles, true);

    // we need this to read settings
    this->setting = new Setting();

    this->proxyManager = std::shared_ptr<ProxyManager>(new ProxyManager(this->setting));


    // we will need this to scan stuff
    this->scanPlayer = new Core::ScanPlayer();

    // initialize db backed models
    this->directoryModel = new Directory();
    this->audiobookFileModel = new AudiobookFile();
    this->audiobookModel = new Audiobook(this->audiobookFileModel, this->proxyManager, this->scanPlayer);

    // model event handlers
    this->audiobookCollectionHandler = std::shared_ptr<AudiobookCollectionHandler>(
            new AudiobookCollectionHandler(this->audiobookModel, this->proxyManager));

    // initialize player, which will initialize vlc backend related items
    this->player = new Core::ConcretePlayer(this->setting, this->proxyManager);

    // initialize the ui
    this->mainWindow = new MainWindow(this->directoryModel,
                                      this->audiobookModel,
                                      this->player,
                                      this->scanPlayer,
                                      this->setting,
                                      this->proxyManager,
                                      this->audiobookCollectionHandler
    );

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
    connect(this->directoryModel, &Directory::directoryRescan,
            this->directoryHandler, &DirectoryHandler::handleDirectoryRescan);


    // we need to register this metatype before using it in signal/slot pattern
    qRegisterMetaType<std::shared_ptr<AudiobookFileProxy>>("std::shared_ptr<AudiobookFileProxy>");
    qRegisterMetaType<QItemSelection>("QItemSelection");

    // set up the events between playerEvents and mainWindow
    connect(this->playerEventHandler, &PlayerEventHandler::notifyPlayerState,
            this->mainWindow, &MainWindow::playerStateUpdated);
    connect(this->playerEventHandler, &PlayerEventHandler::notifyPlayerTime,
            this->mainWindow, &MainWindow::playerTimeUpdated);
    connect(this->playerEventHandler, &PlayerEventHandler::notifyMediaParsed,
            this->mainWindow, &MainWindow::audiobookFileStateUpdated);

    this->scanThread = new QThreadPool();

    std::vector<std::shared_ptr<AudiobookProxy>> list;
    for(int i = 0; i < this->audiobookModel->rowCount(); i++) {
        auto record = this->audiobookModel->record(i);
        auto proxyObject = this->proxyManager->getAudiobookProxy(record);
        list.push_back(proxyObject);
    }

    this->scanThread->start(new InitialScanTask(this->scanPlayer, list));
}

Core::NodokaApp::~NodokaApp() {
}
