//
// Created by mistlight on 1/6/2017.
//

#ifndef NODOKANATIVE_FILEDISPLAYMODEL_H
#define NODOKANATIVE_FILEDISPLAYMODEL_H


#include <QSqlTableModel>

class FileDisplayModel : public QSqlTableModel {

public:
    FileDisplayModel(QObject *parent = 0);
    void setSelectedAudiobook(int audiobookId);

private:
    bool hasFilter;


public slots:
    void selectedAudiobookUpdated(int audiobookId);
};


#endif //NODOKANATIVE_FILEDISPLAYMODEL_H
