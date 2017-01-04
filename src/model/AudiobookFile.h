//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKFILE_H
#define NODOKANATIVE_AUDIOBOOKFILE_H

#include <QSqlTableModel>
#include <memory>
#include <QtCore/QDir>

class AudiobookFile : public QSqlTableModel {

private:
    QMap<int, QString> rowCache;

public:

    AudiobookFile(QObject *parent = 0);
    void registerAudioBook(int audiobookId, std::shared_ptr<QDir> directory);
    void removeAudiobook(int audiobookId);
    void addAudiobookFile(int audiobookId, int position, QString path);


};


#endif //NODOKANATIVE_AUDIOBOOKFILE_H
