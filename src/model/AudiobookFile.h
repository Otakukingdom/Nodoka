//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKFILE_H
#define NODOKANATIVE_AUDIOBOOKFILE_H

#include <QSqlTableModel>
#include <memory>
#include <QtCore/QDir>

class AudiobookFile : public QSqlTableModel {

public:

    AudiobookFile(QObject *parent = 0);
    void registerAudioBook(int audiobookId, std::shared_ptr<QDir> directory);
    int getRowForPath(QString path);

    void addAudiobookFile(QString path);

};


#endif //NODOKANATIVE_AUDIOBOOKFILE_H
