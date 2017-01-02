//
// Created by mistlight on 1/2/2017.
//

#include "NodokaApp.h"

Core::NodokaApp::NodokaApp() {
    // initialize db backed models
    this->directoryModel = new Directory();

    // initialize player, which will initialize vlc backend related items
    this->player = new Core::ConcretePlayer();

    this->mainWindow = new MainWindow(directoryModel);

    // initialize event handlers
    this->directoryHandler = new DirectoryHandler();

    this->setup();
}

void Core::NodokaApp::start() {
    mainWindow->show();
}

void Core::NodokaApp::setup() {
    // set up the listeners for the directory add/remove
    connect(this->directoryModel, &Directory::directoryAdded,
            this->directoryHandler, &DirectoryHandler::handleDirectoryAdded);
    connect(this->directoryModel, &Directory::directoryRemove,
            this->directoryHandler, &DirectoryHandler::handleDirectoryRemoved);
}
