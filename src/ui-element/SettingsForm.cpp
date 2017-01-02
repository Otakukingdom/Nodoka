//
// Created by mistlight on 12/31/2016.
//

#include <QtWidgets/QFileDialog>
#include <QDebug>
#include <QtWidgets/QMessageBox>
#include "SettingsForm.h"

SettingsForm::SettingsForm(Directory* directoryModel, QWidget *parent) :
  QWidget(parent), ui(new Ui::SettingsForm()) {
    ui->setupUi(this);
    ui->listView->setModel(directoryModel);
    ui->listView->setSelectionMode(QAbstractItemView::SelectionMode::SingleSelection);
    this->directoryModel = directoryModel;
    this->setup();
}

SettingsForm::~SettingsForm() {
    delete ui;
}

void SettingsForm::setup() {
    connect(this->ui->addDirectoryButton, &QPushButton::clicked, this, &SettingsForm::performAddDirectory);
    connect(this->ui->removeDirectoryButton, &QPushButton::clicked, this, &SettingsForm::performRemoveDirectory);
    connect(this->ui->closeButton, &QPushButton::clicked, this, &SettingsForm::performClose);
}

void SettingsForm::performAddDirectory() {
    qDebug() << "Perform add dir called";
    auto dir = QFileDialog::getExistingDirectory(this, "Select Folder", "", QFileDialog::ShowDirsOnly);
    qDebug() << "Directory is " << dir;
    this->directoryModel->addDirectory(dir);
}

void SettingsForm::performRemoveDirectory() {
    // get the currently selected item
    auto indexes = this->ui->listView->selectionModel()->selectedIndexes();
    if(indexes.size() == 0) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "You must select a directory to remove");
    } else {
        QModelIndex index = this->ui->listView->selectionModel()->selectedIndexes().first();
        this->directoryModel->removeDirectory(index);
    }
}

void SettingsForm::performClose() {
    this->close();
}
