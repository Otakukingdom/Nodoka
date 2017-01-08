//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_NODOKAAPP_H
#define NODOKANATIVE_NODOKAAPP_H


#include "ConcretePlayer.h"
#include "PlayerEventHandler.h"
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

        PlayerEventHandler* playerEventHandler;

        //private helper function, used to set up the event listeners
        void setup();

    public:
        NodokaApp();
        void start();
    };

}


#endif //NODOKANATIVE_NODOKAAPP_H
