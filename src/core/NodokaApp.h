//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_NODOKAAPP_H
#define NODOKANATIVE_NODOKAAPP_H


#include <QtCore>
#include "ConcretePlayer.h"
#include <src/model/Directory.h>
#include <src/ui-element/MainWindow.h>

namespace Core {
    class NodokaApp : QObject {
    Q_OBJECT
    private:
        ConcretePlayer *player;
        Directory* directoryModel;
        QWidget* mainWindow;

    public:
        NodokaApp();
        void start();


    };

}


#endif //NODOKANATIVE_NODOKAAPP_H
