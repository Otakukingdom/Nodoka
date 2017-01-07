//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_PLAYEREVENTHANDLER_H
#define NODOKANATIVE_PLAYEREVENTHANDLER_H


#include <src/ui-element/MainWindow.h>
#include "ConcretePlayer.h"

namespace Core {
    class PlayerEventHandler : QObject {
    Q_OBJECT

        ConcretePlayer* concretePlayer;
        QWidget* mainWindow;

    public:
        PlayerEventHandler(ConcretePlayer *concretePlayer, QWidget* mainWindow);

        void setupPlayerCallbacks();
    };

}

#endif //NODOKANATIVE_PLAYEREVENTHANDLER_H
