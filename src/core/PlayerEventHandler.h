//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_PLAYEREVENTHANDLER_H
#define NODOKANATIVE_PLAYEREVENTHANDLER_H


#include <src/ui-element/MainWindow.h>
#include "ConcretePlayer.h"

namespace Core {
    class PlayerEventHandler : public QObject {
    Q_OBJECT

        ConcretePlayer* concretePlayer;
        QWidget* mainWindow;

    public:
        PlayerEventHandler(ConcretePlayer *concretePlayer, QWidget* mainWindow);

        void setupPlayerCallbacks();

    signals:
        // when the play state of the media player changes
        void notifyPlayerState(AudiobookFileProxy file, bool isPlaying);

        // when there is progression with the current media file
        void notifyPlayerTime(AudiobookFileProxy file, double currentTime);

        // when media is parsed
        void notifyMediaParsed(AudiobookFileProxy file);

        void notifyPlayerFinished(AudiobookFileProxy file);
    };

}

#endif //NODOKANATIVE_PLAYEREVENTHANDLER_H
