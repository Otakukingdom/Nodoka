//
// Created by mistlight on 12/22/2016.
//

#ifndef NODOKANATIVE_MEDIAPLAYER_H
#define NODOKANATIVE_MEDIAPLAYER_H

#include <memory>
#include "vlc/vlc.h"

namespace Core {
    class ConcretePlayer {

        std::shared_ptr<libvlc_instance_t> inst;
        std::shared_ptr<libvlc_media_player_t> mediaPlayer;
        std::shared_ptr<libvlc_media_t> mediaItem;

    public:
        ConcretePlayer();
    };
}



#endif //NODOKANATIVE_MEDIAPLAYER_H
