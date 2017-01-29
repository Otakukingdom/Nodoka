//
// Created by mistlight on 1/28/2017.
//

#include "ScanPlayer.h"

Core::ScanPlayer::ScanPlayer() {

}

void Core::ScanPlayer::addAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    auto fileList = audiobook->getFilesForAudiobook();
    this->mutex.lock();
    for(int i = 0; i < fileList.size(); i++) {
        this->fileQueue.push(fileList[i]);
    }
    this->mutex.unlock();
}

void Core::ScanPlayer::addAudiobookFile(std::shared_ptr<AudiobookFileProxy> file) {
    this->mutex.lock();
    this->fileQueue.push(file);
    this->mutex.unlock();
}

void scanFile() {

}
