//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MEDIAPLAYER_H
#define NODOKANATIVE_MEDIAPLAYER_H

#include <memory>
#include <QObject>
#include "vlc/vlc.h"

namespace Core {
    class ConcretePlayer : public QObject {
        Q_OBJECT

        libvlc_instance_t* inst;
        libvlc_media_player_t* mediaPlayer;
        libvlc_media_t* mediaItem;

        QString currentPath;

    public:
        void play();
        void loadMedia(QString path);
        void releaseMedia();
        ConcretePlayer();
    };
}



#endif //NODOKANATIVE_MEDIAPLAYER_H
