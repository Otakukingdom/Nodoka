//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookListViewHandler.h"

AudiobookListViewHandler::AudiobookListViewHandler(QMainWindow *window, QListView *audiobookListView) {
    this->mainWindow = window;
    this->audiobookListView = audiobookListView;
}


void AudiobookListViewHandler::handleResetAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {

}

void AudiobookListViewHandler::handleDeleteAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {

}

void AudiobookListViewHandler::contextMenuRequested(const QPoint &position) {
    // we first want to check if user clicked on an audiobook item
    auto modelIndex = this->audiobookListView->indexAt(position);
    if(modelIndex.isValid()) {
        QMenu menu(this->mainWindow);
        menu.addAction("Reset read state");
        menu.addAction("Mark as read");
        menu.addAction("Rescan this Audiobook");
        menu.addAction("Remove this Audiobook");
        menu.exec(this->audiobookListView->mapToGlobal(position));

    } else {
        // if the user clicked somewhere other than the audiobook item, there is nothing to show...
        return;
    }
}

