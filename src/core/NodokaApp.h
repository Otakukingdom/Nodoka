//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_NODOKAAPP_H
#define NODOKANATIVE_NODOKAAPP_H


#include "ConcretePlayer.h"
#include <src/model/Directory.h>
#include <src/ui-element/MainWindow.h>
#include <src/event-handler/DirectoryHandler.h>

namespace Core {

    class NodokaApp : QObject {
    Q_OBJECT
    private:
        ConcretePlayer *player;
        Directory* directoryModel;
        QWidget* mainWindow;
        DirectoryHandler* directoryHandler;

        //private helper function, used to set up the event listeners
        void setup();

    public:
        NodokaApp();
        void start();
    };

}


#endif //NODOKANATIVE_NODOKAAPP_H
