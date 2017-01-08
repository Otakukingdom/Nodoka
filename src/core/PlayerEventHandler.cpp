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
    connect(this->concretePlayer, &ConcretePlayer::stateChanged, [](libvlc_state_t newState) {
        qDebug() << "State changed: " << newState;

        if(libvlc_Playing == newState) {
        }
    });

    connect(this->concretePlayer, &ConcretePlayer::timeProgressed, [](libvlc_time_t time) {
        qDebug() << "Current Time " << time;
    });

}
