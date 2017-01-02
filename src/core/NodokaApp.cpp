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
}

void Core::NodokaApp::start() {
    mainWindow->show();
}
