//
// Created by mistlight on 1/7/17.
//

#include "PlayerEventHandler.h"

Core::PlayerEventHandler::PlayerEventHandler(Core::ConcretePlayer *concretePlayer, QWidget *mainWindow) {
    this->concretePlayer = concretePlayer;
    this->mainWindow = mainWindow;

    this->setupPlayerCallbacks();
}

void Core::PlayerEventHandler::setupPlayerCallbacks() {
    connect(this->concretePlayer, &ConcretePlayer::stateChanged, [this](libvlc_state_t newState) {

        if(libvlc_Playing == newState) {
            notifyPlayerState(*this->concretePlayer->getAudiobookFile(), true);
        } else {
            notifyPlayerState(*this->concretePlayer->getAudiobookFile(), false);
        }
    });

    connect(this->concretePlayer, &ConcretePlayer::timeProgressed, [this](libvlc_time_t time) {
        double currentTime = time / 1000.0;
        notifyPlayerTime(*this->concretePlayer->getAudiobookFile(), currentTime);
    });

}
