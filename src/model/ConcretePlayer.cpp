//
// Created by mistlight on 12/22/2016.
//

#include "ConcretePlayer.h"

ConcretePlayer::ConcretePlayer() {
    /* Load the VLC engine */
    this->inst = libvlc_new (0, NULL);

    return;
}
