import argparse
from pathlib import Path

from fbx import *
from rose.zms import ZMS


def add_zms(scene, zms):
    mesh = FbxMesh.Create(scene, "mesh")

    # TODO: Set control points
    mesh.InitControlPoints(len(zms.vertices))

    for (vertex_idx, vertex) in enumerate(zms.vertices):
        v = FbxVector4(vertex.position.x, vertex.position.z, -vertex.position.y)
        mesh.SetControlPointAt(v, vertex_idx)

    for index in zms.indices:
        mesh.BeginPolygon()
        mesh.AddPolygon(index.x)
        mesh.AddPolygon(index.y)
        mesh.AddPolygon(index.z)
        mesh.EndPolygon()

    mesh.BuildMeshEdgeArray()

    mesh_node = FbxNode.Create(scene, "meshNode")
    mesh_node.SetNodeAttribute(mesh)

    root_node = scene.GetRootNode()
    root_node.AddChild(mesh_node)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("zms", help="ZMS file to convert to fbx")
    args = parser.parse_args()

    input_path = Path(args.zms)
    output_path = input_path.with_suffix("").name

    if not input_path.exists():
        print("ZMS file does not exist")
        return

    manager = FbxManager.Create()

    io_settings = FbxIOSettings.Create(manager, IOSROOT)
    manager.SetIOSettings(io_settings)

    scene = FbxScene.Create(manager, "rose")

    zms = ZMS()
    zms.load(input_path)

    mesh = add_zms(scene, zms)

    exporter = FbxExporter.Create(manager, "")

    res = exporter.Initialize(output_path, -1, manager.GetIOSettings())
    if not res:
        print(f"Failed to initialize exporter: {exporter.GetStatus().GetErrorString()}")
        return

    res = exporter.Export(scene)
    if not res:
        print(f"Failed to export scene: {exporter.GetStatus().GetErrorString()}")
        return

    scene.Destroy()
    exporter.Destroy()
    manager.Destroy()


if __name__ == "__main__":
    main()
